# open-dis-rust
Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation (DIS) application protocol v6 and v7

## Supported PDUs

### DIS v6

| PDU Type | Supported? |
| -------- | ---------- |
| Acknowledge | ✅ |
| AcknowledgeReliable | ✅ |
| ActionRequest | ❌ |
| ActionRequestReliable | ❌ |
| ActionResponse | ❌ |
| ActionResponseReliable | ❌ |
| AggregateState | ❌ |
| ArealObjectState | ❌ |
| CollisionElastic | ❌ |
| Collision | ❌ |
| Comment | ❌ |
| CommentReliable | ❌ |
| CreateEntity | ❌ |
| CreateEntityReliable | ❌ |
| Data | ✅ |
| DataQuery | ✅ |
| DataQueryReliable | ❌ |
| DataReliable | ❌ |
| Designator | ❌ |
| Detonation | ❌ |
| ElectromagneticEmissions | ❌ |
| EntityState | ✅ |
| EntityStateUpdate | ❌ |
| EnvironmentalProcess | ❌ |
| EventReport | ❌ |
| EventReportReliable | ❌ |
| FastEntityState | ❌ |
| Fire | ❌ |
| GriddedData | ❌ |
| IntercomControl | ❌ |
| IntercomSignal | ❌ |
| IsGroupOf | ❌ |
| IsPartOf | ❌ |
| LinearObjectState | ❌ |
| Logistics | ❌ |
| MinefieldData | ❌ |
| MinefieldQuery | ❌ |
| MinefieldResponseNack | ❌ |
| MinefieldState | ❌ |
| PointObjectState | ❌ |
| Receiver | ❌ |
| RecordQueryReliable | ❌ |
| RemoveEntity | ❌ |
| RemoveEntityReliable | ❌ |
| RepairComplete | ❌ |
| RepairResponse | ❌ |
| ResupplyCancel | ❌ |
| ResupplyOffer | ❌ |
| ResupplyReceived | ❌ |
| Sees | ❌ |
| ServiceRequest | ❌ |
| SetData | ❌ |
| SetDataReliable | ❌ |
| SetRecordReliable | ❌ |
| Signal | ❌ |
| StartResume | ❌ |
| StartResumeReliable | ❌ |
| StopFreeze | ❌ |
| StopFreezeReliable | ❌ |
| TransferControlRequest | ❌ |
| Transmitter | ❌ |
| UnderwaterAcoustic | ❌ |
