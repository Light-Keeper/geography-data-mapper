# geography data mapper

Geography data mapper is a simple service that draws datasets on a map.


## Data layout 

```mermaid
erDiagram
    DATASET ||--o{ DATAPOINT : has
    DATAPOINT ||--o{ ATTRIBUTE : has
    
    DATASET {
        string name
    }
    
    DATAPOINT {
        ref dataset_id
        f32 lat
        f32 lng
    }

    ATTRIBUTE {
        ref dataset_id
        ref datapoint_id
        string name
        string value
    }

    GEO_FEATURE ||--o{ GEO_FEATURE_PROPERTY : has

    GEO_FEATURE {
        string type
        string name
        string bbox
        string geometry
    }

    GEO_FEATURE_PROPERTY {
        ref geo_feature_id
        string key
        any value
    }
```


