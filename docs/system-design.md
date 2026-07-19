# System Design Notes

## Proposed Architecture

```
Data Sources
    |
    +-- Manual Inputs
    +-- Chronograph
    +-- Environmental Sensors
    +-- Acoustic Time of Flight
    |
    v
Ballistic Engine
    |
    +-- Trajectory Prediction
    +-- Velocity Modeling
    +-- BC Solver
    |
    v
Analysis Layer
    |
    +-- Confidence Score
    +-- Error Analysis
    +-- Corrections
```

## Measurement Concepts

### Two Point Velocity

Measure velocity near the muzzle and at distance. The velocity loss can be used to estimate drag and BC.

### Acoustic Time of Flight

A microphone system can detect the muzzle report and target impact. With known distance and environmental data, time of flight can constrain BC calculations.

Potential limitations:
- Wind
- Echoes
- Impact detection accuracy
- Unknown projectile behavior

## Hardware Expansion

Possible future hardware:

- Raspberry Pi or embedded controller
- Environmental sensor module
- External microphone
- Camera for impact confirmation

The hardware layer should remain independent from the ballistic calculation engine.
