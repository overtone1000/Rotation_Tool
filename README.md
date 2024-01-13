# Rotation Tool

## Components
- `./core`: rust application for data analysis and rotation auditing
- `./frontend`: svelte web application for displaying rotations
- `./core/data`: data is expected here but is excluded from this repository due to PHI
- `./core/categories`: contains CSVs categorizing exams by code and location

## Mains
- `./core/src/main.rs` contains the main rust application. There are a few modules that can be optionally enabled and disabled. One of them packages the current rotation at `./core/rotations/active.yaml` as well as the `./core/categories` to `./frontend/static` for deployment.
- `./frontend/package.json` contains a script called "build_and_deploy" that can be used to build the frontend and push it to the server.

## To Do
- [ ] Coverage interface does need to update available tree options to make it easier to navigate.
- [ ] Enable selecting "All" in coverage interface? Would need a much more complicated display. Maybe a tree (a lot like the old spreadsheet)
- [ ] Put modalities in exam categorizations so coverage queries don't need to input modality.
- [x] Cached jsons need to be updated if the source is changed. How to do with static files?
- [x] msk wet reads

Board meeting 1/9/24
- [x] Move WVH XR on weekends to OP2 instead of Call 2
- [ ] Add butt-in-chair hours and lunchtimes for each rotation
    - [x] add to rust structs
    - [x] show in frontend
    - [ ] should validate days for hours in backend (that they are valid days with the existing function and that there are no duplicates)
- [x] Show location in frontend
- [ ] Policy section
    - [ ] Butt in chair rule (when someone else is covering, log in to second workstation or at least broadcast a primordial message)
    - [ ] Copy existing policy stuff
- [ ] CT Myelograms