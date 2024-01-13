export interface SiteTree
{
    map:{[key:string]:SubspecialtyTree}
}

export interface SubspecialtyTree{
    map:{[key:string]:ContextTree}
}

export interface ContextTree{
    map:{[key:string]:WeekdayTree}
}

//export interface ModalityTree{
//    map:{[key:string]:WeekdayTree}
//}

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
    exam:string,
    context:string,
    //modality:string,
    dow:number
}

export interface CoordKeys{
    sites:string[],
    exams:string[],
    contexts:string[],
    //modalities:string[]
}

export const getAllCoords = (site_tree:SiteTree):CoordKeys => {

    let site_keys=new Set<string>();
    let exam_keys=new Set<string>();
    let context_keys=new Set<string>();
    let modality_keys=new Set<string>();

    for(const site of Object.keys(site_tree.map))
    {
        site_keys.add(site);
        const exam_tree = site_tree.map[site];
        for(const exam of Object.keys(exam_tree.map))
        {
            exam_keys.add(exam);
            const context_tree = exam_tree.map[exam];
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
        exams:Array.from(exam_keys).sort(),
        contexts:Array.from(context_keys).sort()
        //modalities:Array.from(modality_keys).sort()
    }
}

export const getCoverages = (coords:Coordinates, site_map:SiteTree|undefined):Coverages|undefined=>{
    if(site_map===undefined){return undefined;}

    const exam_map = site_map.map[coords.site];
    if(exam_map===undefined){return undefined;}

    const context_map = exam_map.map[coords.exam];
    if(context_map===undefined){return undefined;}

    const weekday_map = context_map.map[coords.context];
    //if(modality_map===undefined){return undefined;}

    //const weekday_map = modality_map.map[coords.modality];
    if(weekday_map===undefined || weekday_map.map===undefined || weekday_map.map[coords.dow]===undefined){return undefined;}

    return weekday_map.map[coords.dow].coverages;
}