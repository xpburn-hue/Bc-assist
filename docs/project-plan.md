# BC Assist Project Plan

## Purpose

BC Assist is intended to estimate and refine ballistic coefficients using measured data rather than relying only on published values.

## Core Goals

1. Accept firearm, projectile, and environmental inputs.
2. Predict trajectory using standard ballistic models.
3. Compare prediction against measured impacts or velocity data.
4. Solve for a BC value that best matches observed performance.

## Development Roadmap

### Phase 1 - Ballistic Calculator

Inputs:
- Bullet caliber
- Bullet weight
- Muzzle velocity
- Ballistic coefficient estimate
- Temperature
- Pressure
- Humidity
- Elevation

Outputs:
- Drop table
- Velocity decay
- Energy
- Time of flight

### Phase 2 - BC Estimation

Supported measurement methods:

- Velocity measurement at two distances
- Time of flight from muzzle report to target impact
- Known distance and impact data

The solver should account for:
- Atmospheric conditions
- Projectile characteristics
- Measurement uncertainty

### Phase 3 - Sensor Integration

Potential inputs:

- Chronograph data
- Environmental sensors (pressure, temperature, humidity)
- Garmin/Xero data import if available
- Microphone based time-of-flight measurement

### Phase 4 - Advanced Features

- Confidence scoring
- Multiple measurement aggregation
- Camera assisted impact confirmation
- Mil correction suggestions at distance
- Mobile application

## Design Principles

- Favor practical field use.
- Keep components modular.
- Support offline operation.
- Clearly separate measured data from assumptions.
