# Graph Modelling Project

- Fisher Darling \
  11/26/2019 \
  10828285

## 1) Problem Modeling

**a)** The first thing I did was solve the problem without constraints, that is model the movements of the arrows as edges in the graph. For each cell on the board, I constructed a directed graph with that cell as a node and that node's edges all pointing arrows of the opposite color that are along its cardinal direction. This solution had no circles.

Secondly, I augmented the current graph with enough information to allow for circles. To do this I first created two "subgraphs" within the same graph. Each subgraph encodes the transitions from either going forwards (heads) or backwards (tails) from each arrow. So there was a forwards and backwards "graph". Then, every time I was calculating the transitions of a circle, I flipped it's cardinal direction and, rather than connecting to other edges in the same subgraph, I connected them to the corresponding nodes in the opposite subgraph. This linked the graphs together while encoding the idea of forwards or backwards traversal.

By connecting the circles to the opposite subgraph, the search algorithm I used, **A\***, would then be able to make decisions based off of traversing into the opposite subgraph (by ending on a circle) or skipping over it and remaining in the current one.

**b)**

**c)**

I used **A\*** to find the path. But, since my weights we're all the same, and the heuristic was constant, the actual algorithm was essentially dijkstra's (BFS).

**d)**

The algorithm builds every possible state transition, and connects
them fully. If a shortest path does exist, then A\* (BFS in our case), _will always_ find the shortest path since the heuristic is constant and admissible.
