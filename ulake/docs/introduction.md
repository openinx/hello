ULake: A Cloud Native Realtime Lakehouse.
=========================================

## Background

* LakeHouse Basic
    * Share Data
    * Multi Computation Model. (Stream, OLAP, Batch, TP Serving.).
    * Structure, Semi-structure, Un-structure.
* Lakehouse States.
    * Databricks
    * Redshift
    * Iceberg vs Hudi
* Current LakeHouse Shortcomings.
    * Data freshness >= 10min
    * Update unfriendly.
    * Complex
        * Transformation between subseconds real-time scenarios and batch analysis.
        * Consistence is hard to guarantee.
        * Latency & Cost happen.
    * Cost
        * Extra cost to transform.
        * Redudent storage.

## Trendings

* Open Source.
    * Great success of infra software, such as Apache Spark, ElasticSearch, MongoDB, Kafka.
* Cloud Native.
    * Pay as you go.
    * Infinite Capacity - Elasticity
* All-In-One.
    * Integrate different cases into an abstracted system. The classic best-practice:
        * HTAP: Oracle, Google F1/Spanner, PingCAP TiDB/Tiflash. Microsoft SQLServer.
        * HSAP: [Hologres][hologres-url] for big data.
        * [Delta][delta-url]:  Data lake + Data warehouse.
        * Apache Flink: Unified batch stream processing system.
        * Apache Kafka: Queue + Streaming Processing.
* Realtime.
    * Clickhouse/Doris popular trending.
    * Streaming Database popular trending.
        * [Materialize IO][materialize-url]. [7]
        * [Singularity-Data][singularity-data-url].
* Powered by Native Languages (C/C++ or Rust).
    * Close Source
        * [Redshift][redshift-url]
        * Databricks [Photon][photon-url]
    * Open Source
        * Clickhouse, Doris.
        * [Facebook velox][facebook-velox-url].
        * Apache [arrow-ballista][arrow-ballista-url].

[photon-url]: https://cs.stanford.edu/people/matei/papers/2022/sigmod_photon.pdf
[redshift-url]: https://www.amazon.science/publications/amazon-redshift-re-invented
[facebook-velox-url]: https://github.com/facebookincubator/velox
[arrow-ballista-url]: https://github.com/apache/arrow-ballista/
[hologres-url]: https://dl.acm.org/doi/abs/10.14778/3415478.3415550
[delta-url]: https://databricks.com/wp-content/uploads/2020/08/p975-armbrust.pdf
[materialize-url]: https://materialize.com/
[singularity-data-url]: https://singularity-data.com/


## Design
(TODO)

## Roadmap
(TODO)