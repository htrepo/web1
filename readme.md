```
src/
├── main.rs                  # server startup, router wiring
├── lib.rs                   # # re-exports all modules  — fully testable
├── routes/
│   └── mod.rs               # all route definitions
├── handlers/
│   ├── mod.rs
│   └── data_handler.rs      # generate_data handler
├── services/
│   ├── mod.rs
│   └── parquet_service.rs   # write_parquet logic
└── model/
├── mod.rs
└── sales.rs             # Sales struct / schema definition
```
