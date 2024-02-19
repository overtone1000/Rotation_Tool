export interface AnalysisData {
    marks:any[]
};

export interface AnalysisMark {
    weekday:string,
    rvu:number,
    bvu:number,
    rotation:string
}

export function generateColors (number_to_generate:number) {
    let colors:string[] = []
    let huedelta = 1 / number_to_generate;

    let lightness=0.5;
    let saturation=0.5;
  
    for (let i = 0; i < number_to_generate; i++) {
      let hue = i * huedelta
      let rgb=hslToRgb(hue,saturation,lightness);
      let rgb_str="#"+Math.round(rgb[0]).toString(16)+Math.round(rgb[1]).toString(16)+Math.round(rgb[2]).toString(16);
      //console.debug("hsl:",hue,saturation,lightness);
      //console.debug("rgb:",rgb);
      //console.debug("rgbstr:",rgb_str);
      colors.push(rgb_str)
    }
  
    return colors
  }

  /*Converts an HSL color value to RGB. Conversion formula
  * adapted from https://en.wikipedia.org/wiki/HSL_color_space.
  * Assumes h, s, and l are contained in the set [0, 1] and
  * returns r, g, and b in the set [0, 255].
  * */
function hslToRgb(h:number, s:number, l:number) {
    let r:number, g:number, b:number;

    if (s === 0) {
        r = g = b = l; // achromatic
    } else {
        const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
        const p = 2 * l - q;
        r = hueToRgb(p, q, h + 1/3);
        g = hueToRgb(p, q, h);
        b = hueToRgb(p, q, h - 1/3);
    }

    //console.debug("rgb",r,g,b);

    return [r * 255, g * 255, b * 255];
}


function hueToRgb(p:number, q:number, t:number) {
    if (t < 0) t += 1;
    if (t > 1) t -= 1;
    if (t < 1/6) return p + (q - p) * 6 * t;
    if (t < 1/2) return q;
    if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
    return p;
}