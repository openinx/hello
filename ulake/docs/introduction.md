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

## Key Trends

* Open Source.
    * Great success from Infra software, such as:
        * Public List (2022-06-27)
            * Elastic (Value: $7.3 Billion)
            * MongoDB (Vaule: $2 Billion)
            * Confluent (Value: $7 Billion)
            * HashiCorp (Value: $6 Billion)
            * Cloudera (Value: $4.7 Billion)
        * Unlisted
            * OLTP Database
                * PingCAP (Value: $3 Billion)
                * Cockroach Labs (Value: $5 Billion)
                * [Yugabyte][yugabyte-url] (Value: $1.3 Billion)
            * Lakehouse
                * Databricks (Value: $38 Billion)
                * Dremio - Drill + Arrow + Nessie + Iceberg (Value: $2 Billion)
                * Tabular - Apache Iceberg (Raising $11M in series A)
                * Onehouse - Apache Hudi (Raising $8M in seed funding)
            * OLAP Analysis
                * [Starburst][startburst-series-d-url] - Trino (Value: $3.35 Billion)
                * Clickhouse (Value: $2 Billion)
                * Imply - Apache Druid (Value: $1.1 Billion)
                * StarTree - Apache Pinot (Raising $24M in series A)
                * Kyligence - Apache Kylin (Raising $70M in series D)
            * Streaming Processing/Database
                * Materialize - Streaming DB (Raising $60M in series C)
                * StreamNative - MQ + Streaming Processing (Raising $23M in series A)
                * Singularity-Data - Streaming DB (Raising $10M in angel round)
            * AI
                * AnyScale - Ray (Raising $100M in series C, value: $1B)
* Cloud Native.
    * Pay as you go.
    * Infinite Capacity - Elasticity
* All-In-One.
    * Integrate different cases into an abstracted system. The classic best-practice:
        * HTAP: Oracle, Google F1/Spanner, PingCAP TiDB/Tiflash. Microsoft SQLServer.
        * HSAP: [Hologres][hologres-url] for big data.
        * [Delta][delta-url] : data lake + Data warehouse.
        * Apache Flink: Unified batch stream processing system.
        * Apache Kafka: Queue + Streaming Processing.
* Realtime.D
    * Clickhouse/Doris popular trending.
    * Streaming Database popular trending.
        * [Materialize IO][materialize-url].
        * [Singularity-Data][singularity-data-url].
* Powered by Native Languages (C/C++ or Rust).
    * Close Source
        * [Redshift][redshift-url]
        * Databricks [Photon][photon-url]
    * Open Source
        * Clickhouse, Doris.
        * [Facebook velox][facebook-velox-url].
        * Apache [arrow-ballista][arrow-ballista-url].

[yugabyte-url]: https://www.crunchbase.com/organization/yugabyte
[startburst-series-d-url]: https://www.starburst.io/blog/starburst-announces-250m-series-d/
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