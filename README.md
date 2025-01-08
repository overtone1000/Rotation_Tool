# Rotation Tool

## Purpose

This tool was developed to assist in the design and communication of rotation responsibilities to radiologists. Rotations are described in a YAML manifest. The core is a rust application that audits the rotation manifest for errors and generates estimates of rotation volumes using study data. The frontend is a static site generated with svelte that displays the rotation manifest for radiologists and also has a query feature for workflow and technologists to determine who is responsible for a given study.

## Components

- `./core`: rust application for data analysis and rotation auditing
- `./frontend`: svelte web application for displaying rotations
- `./core/data`: data is expected here but is excluded from this repository due to PHI
- `./core/categories`: contains CSVs categorizing exams by code and location

## Methods

### Core

#### Main

The main method of the core application performs the following:
1. Define the the range of dates (`facility_start` and `facility_end`) for which volume data is expected. Currently hard coded.
2. Define the range of dates (`rotation_start` and `rotation_end`) that should be analyzed. This should be narrower than the facility range on the start end as some rotations contain responsibilities that include studies from many days previous. Currently hard coded.
3. Check that volume data is available.
4. Parse the manifest and categorize volume data by rotation and day of the week in a nested tree structure
5. Run any spot tests to check data validity
6. Audit the resulting manifest for errors
    - Coverage overlaps (multiple rotations responsible for the same thing)
    - Coverage gaps (no rotations responsible for something)
    - Coverage of nothing (coverage is defined but no such studies are found in the volume data)
    - Coverage of exams not read by SRC readers (see `Readers.csv` in the Dependencies section)
7. Package the manifest (`./core/rotations/active.yaml`) and categories (`./core/categories`) and generate statistics for the frontend, all of which is stored in `./frontend/static`
8. Run advanced audit

There are a few modules that can be optionally enabled and disabled in the code.

#### Dependencies

`./core/categories` contains several CSVs used by the core.
- `BVU.csv`: Maps exam codes to BVUs
- `Categories_Exam.csv`: Maps exam codes to subspecialties
- `Categories_Location.csv`: Maps locations to contexts
- `Exam_Aliases.csv`: Maps exam codes to aliases (different sites use different exam codes for the same exam)
- `Readers.csv`: A list of readers. Readers who are not SRC radiologists are excluded with the Excluded column.
- `SRC Exam Code Compendium` was used to compile some of the other CSVs and is available for reference.

`./core/rotations` contains the rotation manifest. The core will look for and `active.yml` and an optional `proposed.yml`

### Frontend

`./frontend/package.json` contains a script called "build_and_deploy" that builds the static site and deploys it using a bash script in `./deploy/`. The resulting site structure is:
- `/` (site root) points to the rotation responsibilities and query for the active manifest (`active.yml`)
- `/proposed.html` points to the rotation responsibilities and query for the proposed manifest (`proposed.yml`)
- `/analysis.html` points to the analysis results. 

## To Do

- [ ] Should validate days for hours in backend (that they are valid days with the existing function and that there are no duplicates)
- [ ] Coverage tree improvement
    - Data restructuring
        - The existing coverage tree is a multiply nested HashMap. This structure is okay, but it doesn't handle the "All" case very well because it simply modifies the value for each key, resulting in a large structure with exponentially more key-value pairs than are necessary.
        - Additionally, this is the primary reason that all the actual values for keys (sites, subspecialties, and contexts) need to be known in the code and not just in the source data.
        - A better structure would be a modified map that takes an enum as a key. The enum would have an All, a Vec<String> for multiple keys, and a singular String member for a single key. When the value corresponding to a given key is accessed, the values in All, any Vec<String> containing that key, and the singular key should all be returned as an aggregate
        - An advantage of this approach is that each entry in the rotation manifest would correlate with a single entry in this map.
    - Simplification
        - The existing structure also combines work with coverage. Work should probably be removed, and any structures of interest pertaining to work should be built with helper functions on the map.
        - Similar to combining work and coverage, the audit and analysis functions should be separate.
        - The foreach function is clumsy. Implementing iterator would be superior. __Took a stab at this an implementing an iterator was clumsier.__
- [ ] Need a full exam code reconciliation
- [ ] Exam aliases (and their respective descriptions) aren't in the converage query
