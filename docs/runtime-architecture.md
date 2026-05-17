# Runtime Architecture

## Data Flow

```text
device-core::DeviceRuntime<B: can_core::CanBus>
  -> device-core::PollScheduler
  -> obd-core::ObdClient::request_frame
  -> can_core::CanBus::send
  -> can_core::CanBus::receive
  -> obd-core::ObdClient::parse_response
  -> device-core::DeviceState::update_from_obd_value
  -> device-core::DisplayModel
```

## Current Behavior

- One `step()` sends one request and receives at most one frame.
- Polling sequence is `VehicleSpeed`, `MafAirFlowRate`, repeat.
- No response leaves state unchanged and still returns the current display model.
- Received frames are parsed against the PID requested in the same step.
- CAN errors, OBD parsing errors, and value validation errors are surfaced as `RuntimeError`.

## Display Model Values

- Speed in km/h when PID `0x0D` has been received.
- Fuel rate in L/h when MAF PID `0x10` has been received.
- Cost in EUR/h when fuel rate and fuel price are available.
- Consumption in L/100 km when fuel rate and nonzero speed are available.
- Cost in EUR/100 km when consumption and fuel price are available.

## Out of Scope

- Timing policy
- Retries
- Receive filtering
- Async execution
- Timeout handling
- Display or button drivers
