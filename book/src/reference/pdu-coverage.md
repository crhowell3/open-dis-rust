# PDU Coverage

IEEE 1278.1-2012 defines 72 PDU types. open-dis-rust implements all 72. This
page lists every type with its number, protocol family, Rust type, and status.

| Status | Count | Meaning |
| --- | --- | --- |
| Complete | 55 | Implemented, with no known differences from the standard. |
| Partial | 5 | Implemented, but some variable-length records are kept as raw values or are simplified. |
| Known Issues | 7 | Implemented, but at least one field is encoded differently from the standard. | 
| To Verify | 5 | Implemented, but at least one field may differ from the standard and still needs to be verified. |
| Not Implemented | 0 | Not available yet. |

Each status other than `Complete` links to the details on the [Conformance Notes](conformance.md) page.
`Complete` means no problems are known; it does not mean the PDU has been certified against other implementations.

## List of PDU types

| Type | PDU | Family | Rust type | Status |
| ---: | --- | --- | --- | --- |
| 1 | Entity State | Entity Information | [`EntityStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_information/struct.EntityStatePdu.html) | Complete |
| 2 | Fire | Warfare | [`FirePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/warfare/struct.FirePdu.html) | Complete |
| 3 | Detonation | Warfare | [`DetonationPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/warfare/struct.DetonationPdu.html) | Complete |
| 4 | Collision | Entity Information | [`CollisionPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_information/struct.CollisionPdu.html) | Complete |
| 5 | Service Request | Logistics | [`ServiceRequestPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.ServiceRequestPdu.html) | Complete |
| 6 | Resupply Offer | Logistics | [`ResupplyOfferPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.ResupplyOfferPdu.html) | Complete |
| 7 | Resupply Received | Logistics | [`ResupplyReceivedPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.ResupplyReceivedPdu.html) | Complete |
| 8 | Resupply Cancel | Logistics | [`ResupplyCancelPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.ResupplyCancelPdu.html) | Complete |
| 9 | Repair Complete | Logistics | [`RepairCompletePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.RepairCompletePdu.html) | Complete |
| 10 | Repair Response | Logistics | [`RepairResponsePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/logistics/struct.RepairResponsePdu.html) | Complete |
| 11 | Create Entity | Simulation Mgmt | [`CreateEntityPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.CreateEntityPdu.html) | Complete |
| 12 | Remove Entity | Simulation Mgmt | [`RemoveEntityPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.RemoveEntityPdu.html) | Complete |
| 13 | Start/Resume | Simulation Mgmt | [`StartResumePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.StartResumePdu.html) | Complete |
| 14 | Stop/Freeze | Simulation Mgmt | [`StopFreezePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.StopFreezePdu.html) | Complete |
| 15 | Acknowledge | Simulation Mgmt | [`AcknowledgePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.AcknowledgePdu.html) | Complete |
| 16 | Action Request | Simulation Mgmt | [`ActionRequestPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.ActionRequestPdu.html) | Complete |
| 17 | Action Response | Simulation Mgmt | [`ActionResponsePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.ActionResponsePdu.html) | Complete |
| 18 | Data Query | Simulation Mgmt | [`DataQueryPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.DataQueryPdu.html) | [Known issues](conformance.md#data-query-pdu) |
| 19 | Set Data | Simulation Mgmt | [`SetDataPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.SetDataPdu.html) | Complete |
| 20 | Data | Simulation Mgmt | [`DataPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.DataPdu.html) | Complete |
| 21 | Event Report | Simulation Mgmt | [`EventReportPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.EventReportPdu.html) | Complete |
| 22 | Comment | Simulation Mgmt | [`CommentPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management/struct.CommentPdu.html) | Complete |
| 23 | Electromagnetic Emission | Distributed Emission | [`ElectromagneticEmissionsPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/distributed_emissions/struct.ElectromagneticEmissionsPdu.html) | [Known issues](conformance.md#electromagnetic-emission-pdu) |
| 24 | Designator | Distributed Emission | [`DesignatorPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/distributed_emissions/struct.DesignatorPdu.html) | Complete |
| 25 | Transmitter | Radio | [`TransmitterPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/radio_communications/struct.TransmitterPdu.html) | Complete |
| 26 | Signal | Radio | [`SignalPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/radio_communications/struct.SignalPdu.html) | Complete |
| 27 | Receiver | Radio | [`ReceiverPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/radio_communications/struct.ReceiverPdu.html) | Complete |
| 28 | IFF | Distributed Emission | [`IFFPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/distributed_emissions/struct.IFFPdu.html) | [Partial](conformance.md#iff-pdu) |
| 29 | Underwater Acoustic | Distributed Emission | [`UnderwaterAcousticPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/distributed_emissions/struct.UnderwaterAcousticPdu.html) | [Known issues](conformance.md#underwater-acoustic-pdu) |
| 30 | Supplemental Emission | Distributed Emission | [`SupplementalEmissionPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/distributed_emissions/struct.SupplementalEmissionPdu.html) | Complete |
| 31 | Intercom Signal | Radio | [`IntercomSignalPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/radio_communications/struct.IntercomSignalPdu.html) | Complete |
| 32 | Intercom Control | Radio | [`IntercomControlPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/radio_communications/struct.IntercomControlPdu.html) | [Partial](conformance.md#intercom-control-pdu) |
| 33 | Aggregate State | Entity Mgmt | [`AggregateStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_management/struct.AggregateStatePdu.html) | [Partial](conformance.md#aggregate-state-pdu) |
| 34 | Is Group Of | Entity Mgmt | [`IsGroupOfPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_management/struct.IsGroupOfPdu.html) | [Partial](conformance.md#is-group-of-pdu) |
| 35 | Transfer Ownership | Entity Mgmt | [`TransferOwnershipPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_management/struct.TransferOwnershipPdu.html) | [To verify](conformance.md#record-specification-record) |
| 36 | Is Part Of | Entity Mgmt | [`IsPartOfPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_management/struct.IsPartOfPdu.html) | Complete |
| 37 | Minefield State | Minefield | [`MinefieldStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/minefield/struct.MinefieldStatePdu.html) | Complete |
| 38 | Minefield Query | Minefield | [`MinefieldQueryPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/minefield/struct.MinefieldQueryPdu.html) | Complete |
| 39 | Minefield Data | Minefield | [`MinefieldDataPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/minefield/struct.MinefieldDataPdu.html) | Complete |
| 40 | Minefield Response NACK | Minefield | [`MinefieldResponseNackPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/minefield/struct.MinefieldResponseNackPdu.html) | [Known issues](conformance.md#minefield-response-nack-pdu) |
| 41 | Environmental Process | Synthetic Env | [`EnvironmentalProcessPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/synthetic_environment/struct.EnvironmentalProcessPdu.html) | [Partial](conformance.md#environmental-process-pdu) |
| 42 | Gridded Data | Synthetic Env | [`GriddedDataPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/synthetic_environment/struct.GriddedDataPdu.html) | [Known issues](conformance.md#gridded-data-pdu) |
| 43 | Point Object State | Synthetic Env | [`PointObjectStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/synthetic_environment/struct.PointObjectStatePdu.html) | Complete |
| 44 | Linear Object State | Synthetic Env | [`LinearObjectStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/synthetic_environment/struct.LinearObjectStatePdu.html) | [Known issues](conformance.md#linear-object-state-pdu) |
| 45 | Areal Object State | Synthetic Env | [`ArealObjectStatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/synthetic_environment/struct.ArealObjectStatePdu.html) | Complete |
| 46 | Time Space Position Information (TSPI) | Live Entity | [`TimeSpacePositionInformationPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/live_entity_information/struct.TimeSpacePositionInformationPdu.html) | Complete |
| 47 | Appearance | Live Entity | [`AppearancePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/live_entity_information/struct.AppearancePdu.html) | Complete |
| 48 | Articulated Parts | Live Entity | [`ArticulatedPartsPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/live_entity_information/struct.ArticulatedPartsPdu.html) | Complete |
| 49 | Live Entity Fire | Live Entity | [`LiveEntityFirePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/live_entity_information/struct.LiveEntityFirePdu.html) | [To verify](conformance.md#live-entity-fire-and-detonation-pdus) |
| 50 | Live Entity Detonation | Live Entity | [`LiveEntityDetonationPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/live_entity_information/struct.LiveEntityDetonationPdu.html) | [To verify](conformance.md#live-entity-fire-and-detonation-pdus) |
| 51 | Create Entity-R | Simulation Mgmt-R | [`CreateEntityReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.CreateEntityReliablePdu.html) | Complete |
| 52 | Remove Entity-R | Simulation Mgmt-R | [`RemoveEntityReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.RemoveEntityReliablePdu.html) | Complete |
| 53 | Start/Resume-R | Simulation Mgmt-R | [`StartResumeReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.StartResumeReliablePdu.html) | Complete |
| 54 | Stop/Freeze-R | Simulation Mgmt-R | [`StopFreezeReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.StopFreezeReliablePdu.html) | Complete |
| 55 | Acknowledge-R | Simulation Mgmt-R | [`AcknowledgeReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.AcknowledgeReliablePdu.html) | Complete |
| 56 | Action Request-R | Simulation Mgmt-R | [`ActionRequestReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.ActionRequestReliablePdu.html) | [To verify](conformance.md#action-request-r-pdu) |
| 57 | Action Response-R | Simulation Mgmt-R | [`ActionResponseReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.ActionResponseReliablePdu.html) | Complete |
| 58 | Data Query-R | Simulation Mgmt-R | [`DataQueryReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.DataQueryReliablePdu.html) | Complete |
| 59 | Set Data-R | Simulation Mgmt-R | [`SetDataReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.SetDataReliablePdu.html) | Complete |
| 60 | Data-R | Simulation Mgmt-R | [`DataReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.DataReliablePdu.html) | Complete |
| 61 | Event Report-R | Simulation Mgmt-R | [`EventReportReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.EventReportReliablePdu.html) | Complete |
| 62 | Comment-R | Simulation Mgmt-R | [`CommentReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.CommentReliablePdu.html) | Complete |
| 63 | Record-R | Simulation Mgmt-R | [`RecordReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.RecordReliablePdu.html) | Complete |
| 64 | Set Record-R | Simulation Mgmt-R | [`SetRecordReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.SetRecordReliablePdu.html) | [To verify](conformance.md#record-specification-record) |
| 65 | Record Query-R | Simulation Mgmt-R | [`RecordQueryReliablePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/simulation_management_with_reliability/struct.RecordQueryReliablePdu.html) | Complete |
| 66 | Collision-Elastic | Entity Information | [`CollisionElasticPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_information/struct.CollisionElasticPdu.html) | Complete |
| 67 | Entity State Update | Entity Information | [`EntityStateUpdatePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_information/struct.EntityStateUpdatePdu.html) | Complete |
| 68 | Directed Energy Fire | Warfare | [`DirectedEnergyFirePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/warfare/struct.DirectedEnergyFirePdu.html) | [Known issues](conformance.md#directed-energy-fire-pdu) |
| 69 | Entity Damage Status | Warfare | [`EntityDamageStatusPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/warfare/struct.EntityDamageStatusPdu.html) | Complete |
| 70 | Information Operations Action | Information Ops | [`InformationOperationsActionPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/information_operations/struct.InformationOperationsActionPdu.html) | Complete |
| 71 | Information Operations Report | Information Ops | [`InformationOperationsReportPdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/information_operations/struct.InformationOperationsReportPdu.html) | Complete |
| 72 | Attribute | Entity Information | [`AttributePdu`](https://docs.rs/open-dis-rust/latest/open_dis_rust/entity_information/struct.AttributePdu.html) | Complete |

The family column uses short names. Each Rust type lives in the module for its family; for example,
`open_dis_rust::warfare::FirePdu`. PDU type 0 (Other) is a placeholder in the enumeration, and there
is no corresponding PDU.

## Headers

Two header types are used:

- **`PduHeader`** for every family except Live Entity.
- **`LiveEntityPduHeader`** for the five Live Entity PDUs (types 46-50). It has the same 12-octet size,
  but it carries a subprotocol number where the PDU Status record would be.

Both are covered on the [Wire Format Conventions](wire-format.md) page.
