# Runtime Architecture

## Data Flow

```text
CAN bus
  -> can-core::CanFrame
  -> obd-core::ObdClient
  -> obd-core::Mode01Value
  -> device-core::DeviceState
  -> device-core::DisplayModel
  -> display driver
