export interface SiteTree
{
    map:{[key:string]:SubspecialtyTree}
}

export interface SubspecialtyTree{
    map:{[key:string]:ContextTree}
}

export interface ContextTree{
    map:{[key:string]:ModalityTree}
}

export interface ModalityTree{
    map:{[key:string]:WeekdayTree}
}

export interface WeekdayTree{
    map:{[key:number]:{coverages:Coverages}}
}

export interface Coverages {
    Temporal?:TemporalCoverage[],
    Fractional?:FractionalCoverage[],
}

export interface TemporalCoverage{
    start:MinutesSinceMidnight,
    end:MinutesSinceMidnight,
    rotation:string,
    rotation_day:number
}

export interface MinutesSinceMidnight
{
    minutes:number
}

export interface FractionalCoverage{
    rotation:string,
    rotation_day:number,
    fraction:number
}

export interface Coordinates{
    site:string,
    subspecialty:string,
    context:string,
    modality:string,
    dow:number
}

export interface CoordKeys{
    sites:string[],
    subspecialties:string[],
    contexts:string[],
    modalities:string[]
}

export const getAllCoords = (site_tree:SiteTree):CoordKeys => {

    let site_keys=new Set<string>();
    let subspecialty_keys=new Set<string>();
    let context_keys=new Set<string>();
    let modality_keys=new Set<string>();

    for(const site of Object.keys(site_tree.map))
    {
        site_keys.add(site);
        const subspecialty_tree = site_tree.map[site];
        for(const subspecialty of Object.keys(subspecialty_tree.map))
        {
            subspecialty_keys.add(subspecialty);
            const context_tree = subspecialty_tree.map[subspecialty];
            for(const context of Object.keys(context_tree.map))
            {
                context_keys.add(context);
                const modality_tree = context_tree.map[context];
                for(const modality of Object.keys(modality_tree.map))
                {
                    modality_keys.add(modality);
                }   
            }   
        }   
    }

    
    return {
        sites:Array.from(site_keys).sort(),
        subspecialties:Array.from(subspecialty_keys).sort(),
        contexts:Array.from(context_keys).sort(),
        modalities:Array.from(modality_keys).sort()
    }
}

export const getCoverages = (coords:Coordinates, site_map:SiteTree|undefined):Coverages|undefined=>{
    if(site_map===undefined){return undefined;}

    const subspecialty_map = site_map.map[coords.site];
    if(subspecialty_map===undefined){return undefined;}

    const context_map = subspecialty_map.map[coords.subspecialty];
    if(context_map===undefined){return undefined;}

    const modality_map = context_map.map[coords.context];
    if(modality_map===undefined){return undefined;}

    const weekday_map = modality_map.map[coords.modality];
    if(weekday_map===undefined || weekday_map.map===undefined || weekday_map.map[coords.dow]===undefined){return undefined;}

    return weekday_map.map[coords.dow].coverages;
}