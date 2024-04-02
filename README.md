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
- [ ] Should validate days for hours in backend (that they are valid days with the existing function and that there are no duplicates)
- [ ] Copy existing policy stuff
- [ ] Coverage tree improvement
    - Data restructuring
        - __Took a stab at this and got lost in complicated generics and enums.__
        - The existing coverage tree is a multiply nested HashMap. This structure is okay, but it doesn't handle the "All" case very well because it simply modifies the value for each key, resulting in a large structure with exponentially more key-value pairs than are necessary.
        - Additionally, this is the primary reason that all the actual values for keys (sites, subspecialties, and contexts) need to be known in the code and not just in the source data.
        - A better structure would be a modified map that takes an enum as a key. The enum would have an All, a Vec<String> for multiple keys, and a singular String member for a single key. When the value corresponding to a given key is accessed, the values in All, any Vec<String> containing that key, and the singular key should all be returned as an aggregate
        - An advantage of this approach is that each entry in the rotation manifest would correlate with a single entry in this map.
    - Simplification
        - The existing structure also combines work with coverage. Work should probably be removed, and any structures of interest pertaining to work should be built with helper functions on the map.
        - Similar to combining work and coverage, the audit and analysis functions should be separate.
        - The foreach function is clumsy. Implementing iterator would be superior. __Took a stab at this an implementing an iterator was clumsier.__