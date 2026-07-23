# Solver Validation Framework

This directory contains validation infrastructure for the ballistic solver.

## Goals

- Prevent silent trajectory changes during future development.
- Compare solver output against trusted reference trajectories.
- Verify numerical stability when integration parameters change.
- Provide repeatable performance measurements.

## Golden Fixtures

Reference trajectory datasets should include:

- muzzle velocity
- projectile properties
- drag model
- environment
- trajectory samples
- expected tolerances

Fixtures should be added whenever a new ballistic feature changes solver behavior.

## Validation Philosophy

The solver remains deterministic and internally uses imperial units. Validation compares the public solver output without introducing unit conversions into the integration loop.
