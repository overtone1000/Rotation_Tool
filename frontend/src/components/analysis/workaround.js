//This works but typescript complains so put it in raw javascript to circumvent
export let workaround =
{
    y: "value",
    fx: "rotation",
    inset: 0,    
}

export let workaround_tips =
{
    y: "value", //This pulls raw data into y channel for subseequent transformation
    fx: "rotation", //This pulls raw data into fx channel for subsequent transformation
    //members: "members",   
    stroke: "green",
    fill: "white",
    
    //title: (d)=>{return "Hello";},

    format: {
        fx:true,
        fill:true,
        count:false,
        members:true,
        identity:false,
        y:false
    }
}