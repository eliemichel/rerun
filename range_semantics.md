TODO







Cacheable aggregated query semantics
====================================

We want to be able to cache aggregations of Chunks, regardless of how these chunks came together.
This will be a very important part of our upcoming Chunk processing primitives:
* https://github.com/rerun-io/rerun/issues/8221

Specifically, we want to be able to cache Chunks that were _range-zipped_ together.
[Range-zipping is a well-defined, deterministic operation](https://github.com/rerun-io/rerun/blob/main/crates/store/re_query/src/range_zip/generated.rs) that we do all over the place, any time we need to interlace data from multiple streams together.
It is the core joining operation in Rerun: grab columnar data from different components, and zip them together over time (where "time" really means "index" and is defined as a tuple of `(Timeline, RowId)`).

Unfortunately, as of today, range-zipping seemingly behaves differently depending on how the data was queried: `LatestAt` vs. `Range`.
How is that even possible that the origin of the data impacts this operation at all, and what can we do to get out of this mess?

This is a pre-requisite for:
* TODO: link to detailed chunk processor caching proposal


## Background: range-zipping

Say you have the following two chunks:
```rust
  frame_nr   component
  --------   ---------

  CHUNK C0
    #0       Radius(1.0)
    #15      Radius(2.0)

  CHUNK C1
    #10      Position(1, 1, 1)
    #20      Position(1, 2, 1)
```

Range-zipping is the operation that will merge these two streams of data into a single logical stream.
It is a form of aggregation, and it used by all of our visualizers in order to merge the many different streams of components they get from executing `LatestAt` and/or `Range` queries.

Range-zipping requires a point-of-view, from which the joining is performed. The resulting stream of data will yield one entry for each index where the point-of-view was updated, in any of the source streams.
As of today, we only ever rely on single point-of-views, though that is expected to change in the future.

There are two unique components in this specific example, and as such there are two possible range-zip operations possible:
* `RangeZip(PoV: Position, chunks: [C0, C1])`
* `RangeZip(PoV: Color,    chunks: [C0, C1])`

`RangeZip(PoV: Position, chunks: [C0, C1])` yields:
```rust
CHUNK(RangeZip(PoV: Position, chunks: [C0, C1]))
  frame_nr    position (pov)      radius
  --------    --------------      ------
    #10       Position(1, 1, 1)   Radius(1.0)
    #20       Position(1, 2, 1)   Radius(2.0)
```

`RangeZip(PoV: Color, chunks: [C0, C1])` yields:
```rust
CHUNK(RangeZip(PoV: Color, chunks: [C0, C1]))
  frame_nr    radius        position (pov)
  --------    ------        --------------
    #0        Radius(1.0)   <None>
    #15       Radius(2.0)   Position(1, 1, 1)
```


### :warning: Pitfall: Range-zipping is offset sensitive!

Range-zipping is *not* a globally deterministic computation -- it can and will yield contradicting results if the data streams get offset or filtered in any way.

Consider the two Chunks from the example above once again, but this time let's assume they have been applied a `.filtered(#5..=#20)`:
```rust
  frame_nr   component
  --------   ---------

  CHUNK C0
    #15      Radius(2.0)

  CHUNK C1
    #10      Position(1, 1, 1)
    #20      Position(1, 2, 1)
```

`RangeZip(PoV: Position, chunks: [C0, C1])` now yields:
```diff
CHUNK(RangeZip(PoV: Position, chunks: [C0, C1]))
  frame_nr    position (pov)      radius
  --------    --------------      ------
-   #10       Position(1, 1, 1)   Radius(1.0)
+   #10       Position(1, 1, 1)   <None>
    #20       Position(1, 2, 1)   Radius(2.0)
```

The same operation on the same Chunks yielded the same entries (indices `#10` and `#20`), but with different data!

This is the source of a lot of complexity, and has caused numerous bugs, particularly around caching.
The obvious fix is to always apply range-zipping on raw, unaltered Chunks... but as we'll see, things actually aren't that simple in practice, as query semantics get involved.


## Background: aggregated queries

We define an *aggregated query* as a series of single-component `LatestAt` and/or `Range` queries (reminder: there is no such thing as a multi-component query), whose results (Chunks) are then aggregated using range-zipping.

To avoid confusion, I will refer to single-component/low-level queries as `LatestAt` and `Range`, whereas multi-component/aggregated/range-zipped/high-level queries will be referred to as `AggregatedLatestAt` and `AggregatedRange`, respectively.

Aggregated queries are the core building block of our visualizers:
* Because they rely on range-zipping, they inherit its "offset sensitiveness" property.
* Because they rely on `LatestAt` and `Range` queries, they also inherit their semantic quirks.
* Because they mix all of these things, they also come with their very own annoying pitfalls.

In particular, there are two very specific aggregated-query semantic pitfalls that make it impossible to cache range-zipped Chunks at the moment:
* Local vs. global determinism
* Peeking into the future


### :warning: Pitfall: Local vs. global determinism

Our `AggregatedLatestAt` queries are globally deterministic, whereas our `AggregatedRange` queries are merely locally deterministic.

What that means in pratice is that for a given _fixed, immutable dataset_ (i.e. no Chunks are actively being added nor removed from the store), the results you get at timestamp `ts_result`, for a `LatestAt` query at timestamp `ts_query`, are completely deterministic, regardless of what `ts_query` you use.
This is _not_ true for `Range` queries.

Say you have the following data residing in storage:
```rust
CHUNK_STORE
  frame_nr   component
  --------   ---------

  CHUNK C0
    #0       Radius(1.0)
    #15      Radius(2.0)

  CHUNK C1
    #10      Position(1, 1, 1)
    #20      Position(1, 2, 1)
```

Any `AggregatedLatestAt` query executed for `#10 <= t < #15` will always yield the same results:
* `AggregatedLatestAt(at: #10, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(1.0))`
* `AggregatedLatestAt(at: #11, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(1.0))`
* `AggregatedLatestAt(at: #12, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(1.0))`
* `AggregatedLatestAt(at: #13, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(1.0))`
* `AggregatedLatestAt(at: #14, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(1.0))`

Now consider an `AggregatedRange` query -- it only takes two examples to see it all fall apart:
* `AggregatedRange(range: #0..#12, PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), Radius(1.0))`]
* `AggregatedRange(range: #1..#12, PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), None)`]

Both `AggregatedRange` queries yield data from the same index (`#10`), but with different values: `AggregatedLatestAt` queries are globally deterministic, whereas `AggregatedRange` queries are locally deterministic.
This is very much in the lineage of range-zipping's offset sensitiveness.

This is a natural and easy to understand consequence of `AggregatedRange` queries not being bootstrapped -- but you can extrapolate the effect that something like this has on caching.
In fact, this is one of the main reason why our range-query cache doesn't actually cache queries at all, but rather the underlying Chunks necessary to compute the results: caching actual range queries would be extremely painful (we know, we've been there).


#### Why is that a problem for caching range-zipped Chunks?

Consider what happens when range-zipping the two Chunks above:
```rust
CHUNK(RangeZip(PoV: Position, chunks: [C0, C1]))
  frame_nr   position (pov)      radius
  --------   --------------      ------
    #10      Position(1, 1, 1)   Radius(1.0)
    #20      Position(1, 2, 1)   Radius(2.0)
```

Notice that [`#10: (Position(1, 1, 1), None)`], which is a possible outcome of running a `Range` query on the dataset, is not a possible value when blindly zipping the Chunks together without further context.
This in turn makes aggregated caching impossible.


### :warning: Pitfall: Peeking into the (intra-timestamp) future

`AggregatedLatestAt` queries break the rules of indexing by peeking into the future, whereas our `AggregatedRange` queries don't.

Say you have the following data residing in storage:
```rust
CHUNK_STORE
  frame_nr   row_id   component
  --------   ------   ---------

  CHUNK C0
    #0       101      Radius(1.0)
    #10      1099     Radius(2.0)

  CHUNK C1
    #10      1001     Position(1, 1, 1)
    #20      2001     Position(1, 2, 1)
```

If you were to run a `AggregatedLatestAt` query on top of that data at `t=#10`, you'd get the following (hint: take a close look at the `Radius`):
* `AggregatedLatestAt(at: #10, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(2.0))`

Compare that to an `AggregatedRange` query (hint: look at the `Radius`):
* `AggregatedRange(range: #0..#11, PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), Radius(1.0))`]

What the `AggregatedLatestAt` query is doing is technically illegal: somehow we're saying that our `Position` at index `(#10, 1001)` has an associated `Radius` at index `(#10, 1099)` -- that is, from the future.

Of course this is no mistake, the viewer would be pretty much unusable otherwise (imagine having to meticulously execute your log calls in the perfect order when trying to log multiple components to the same `frame_nr`).
This is very much intended behavior, and the only reason it works at all is because we explicitly monkey-patch the `RowId`s at the last second in the visualizers to let them think that the data is _not_ coming from the future:
https://github.com/rerun-io/rerun/blob/94d545b52bc8039332281c17c8b5773140caff49/crates/viewer/re_space_view/src/results_ext.rs#L368-L373


#### Why is that a problem for caching range-zipped Chunks?

Consider what happens when range-zipping the two Chunks above:
```rust
CHUNK(RangeZip(PoV: Position, chunks: [C0, C1]))
  frame_nr   position (pov)      radius
  --------   --------------      ------
    #10       Position(1, 1, 1)   Radius(1.0)
    #20       Position(1, 2, 1)   Radius(2.0)
```

Notice that `#10: (Position(1, 1, 1), Radius(2.0))`, which is a possible outcome of running an `AggregatedLatestAt` query on the dataset, is not a possible value when blindly zipping the Chunks together without further context.
This in turn makes aggregated caching impossible.


TODO: I guess we don't need to solve this one at this layer, right?


---

Both of these pitfalls result in the same problem: they violate the semantics of range-zipping in one way or another, which in turn makes it impossible to cache range-zipped chunks.

TODO: do they?


## Proposal

The proposal is to:
1. Keep the semantics of `LatestAt` and `Range` exactly as-is.
2. Keep the semantics of `AggregatedLatestAt` exactly as-is.
3. Tweak the semantics of `AggregatedRange` to make them closer to those of `AggregatedLatestAt`.


Specifically, we want to make it so that `Range` queries globally bootstrap their secondary components (and only their secondary components), and do so _with_ future-peeking semantics. 


Say we have this store:
```rust
CHUNK_STORE
  frame_nr   row_id   component
  --------   ------   ---------

  CHUNK C0
    #0       101      Radius(1.0)
    #10      1099     Radius(2.0)

  CHUNK C1
    #10      1001     Position(1, 1, 1)
    #20      2001     Position(1, 2, 1)
```

This what a `Range(range: #0..=#20)` actually sees now:
```rust
CHUNK_STORE
  frame_nr   row_id   component
  --------   ------   ---------

  CHUNK C0(LatestAt(#0), MONKEY-PATCHED, FLATTENED?)
    #0       0        Radius(1.0)

  CHUNK C0
    #0       101      Radius(1.0)
    #10      1099     Radius(2.0)

  CHUNK C1
    #10      1001     Position(1, 1, 1)
    #20      2001     Position(1, 2, 1)
```

More generally, every `Range` query starting at `t0` will virtually include the monkey-patch `LatestAt` results for all secondary components at `t0`.


* `LatestAt(at: #0,  comps: [Position, Radius])` = `None` (no position, we discard)
* `LatestAt(at: #5,  comps: [Position, Radius])` = `None` (no position, we discard)
* `LatestAt(at: #10, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(2.0))`
* `LatestAt(at: #15, comps: [Position, Radius])` = `#10: (Position(1, 1, 1), Radius(2.0))`
* `LatestAt(at: #20, comps: [Position, Radius])` = `#20: (Position(1, 2, 1), Radius(2.0))`

* `Range(range: #0..#11,  PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), Radius(1.0))`]
  * `LatestAt(at: #0,  comps: [Radius])` = `#0:  (Radius(1.0))`
    ```rust
    CHUNK_STORE
      frame_nr   row_id   component

      CHUNK C0(LatestAt(#0), MONKEY-PATCHED, FLATTENED?)
        #0       0        Radius(1.0)

    // [...]
    ```rust
* `Range(range: #5..#11,  PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), Radius(1.0))`]
  * `LatestAt(at: #5,  comps: [Radius])` = `#5:  (Radius(1.0))`
    ```rust
    CHUNK_STORE
      frame_nr   row_id   component

      CHUNK C0(LatestAt(#0), MONKEY-PATCHED, FLATTENED?)
        #0       0        Radius(1.0)

    // [...]
    ```
* `Range(range: #10..#11, PoV: Position, comps: [Radius])` = [`#10: (Position(1, 1, 1), Radius(1.0))`]
  * `LatestAt(at: #10, comps: [Radius])` = `#10: (Radius(1.0))`
    ```rust
    CHUNK_STORE
      frame_nr   row_id   component

      CHUNK C0(LatestAt(#0), MONKEY-PATCHED, FLATTENED?)
        #0       0        Radius(1.0)

    // [...]
    ```

    How does this compare to range-zipping raw chunks no?


Say we do this at the `re_query` layer, by introducing a new `bootstrapped_range` query helper. It doesn't even need a cache or anything, it just relies on the existing `LatestAt` and `Range` caches (yes, both!).
Dependency-tracking wise, it "just works", there's no particular extra logic needed.


#### How does that help with caching range-zipped Chunks?

With these changes, 

TODO: reuse examples from above


TODO: isn't peeking into the future still a problem though? (edit: no?)
TODO: `LatestAt(#10) ~= Range(#10..=#10)` now!
  TODO: 2 differences remaining after: intra-timestamp yields, bootstrapped primaries
TODO: relation to dataframe semantics
TODO: impact on other things?
TODO: this document not mentioning dataframes seems very weird surely
