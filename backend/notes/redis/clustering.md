# clustering

Redis Cluster provides a way to run a Redis installation where data is automatically sharded across multiple Redis nodes.

- Automatically split your dataset among multiple nodes.
- Continue operations when a subset of the nodes are experiencing failures or are unable to communicate with the rest of the cluster.

## Every Redis Cluster node requires two open TCP connections

- Redis TCP port used to serve clients, e.g., 6379,
- second port known as the cluster bus port 16379.
  - the cluster bus port is set by adding 10000 to the data port (e.g., 16379)
  - Cluster bus is a node-to-node communication channel that uses a binary protocol,which is more suited to exchanging information between nodes due to little bandwidth and processing time
  - Nodes use the cluster bus for failure detection, configuration updates, failover authorization, and so forth.
  - Clients should never try to communicate with the cluster bus port, but rather use the Redis command port.
  - However, make sure you open both ports in your firewall, otherwise Redis cluster nodes won't be able to communicate.
