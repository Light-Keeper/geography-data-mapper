# geography data mapper

Geography data mapper is a simple service that draws datasets on a map.


## Data layout 

```mermaid
erDiagram
    datasets ||--|{ datapoints : dataset_id
    datapoints ||--|{ attributes : datapoint_id
    datasets ||--|{ attributes : dataset_id

    datasets {
        int id PK
        string name
    }

    datapoints {
        int id PK
        int dataset_id FK
        float lng
        float lat
    }
    
    attributes {
        int id PK
        int dataset_id FK
        int datapoint_id FK
        string name
        float value
    }
```


