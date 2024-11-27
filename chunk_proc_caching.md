



Caching aggregated Chunks
=========================

We've had a long discussion with @Wumpf specifically regarding the caching aspect.
Chunk Processors will require us to cache aggregated data (range-zipped data, to be exact), which is a first for us.

For the following proposal to work, this would first require new aggregated query semantics:
* TODO: lint to aggregated query thing


TODO: the semantic proposal introduces a distinction between `LatestAt`/`Range` and `AggregatedLatestAt`/`AggregatedRange`. This proposal will need to introduce yet another concept -> `UnslicedAggregatedLatestAt`/`UnslicedAggregatedRange`.


### Query example

TODO: first and foremost, queries don't return slices anymore, they return whole Chunks, always
TODO: we need access to queries that do not even pre-slice chunks



TODO: GC events don't matter, just LRU everything, for the same reasons as in the query cache.
> This data can be recreated at any point using the input chunk and is invalidated exactly when the chunk is removed from the store. 






TODO: This is all about dependency tracking
TODO: all very similar to the query cache, see my other comment in this thread.


TODO: is there anything we should be worrying about storage-node wise or whatever?











