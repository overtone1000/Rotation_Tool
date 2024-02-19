use serde::Serialize;

#[derive(Serialize)]
pub struct Plot<T>
where T:Serialize {
    marks:Vec<T>
}

impl <T> Plot<T>
where T:Serialize {
    pub fn new()->Self{
        Self{
            marks:Vec::new()
        }
    }
    pub fn push(&mut self, newmember:T)->(){
        self.marks.push(newmember);
    }
}

#[derive(Serialize)]
pub struct AnalysisMark<'a>
{
    pub weekday:chrono::Weekday,
    pub rvu:f64,
    pub bvu:f64,
    pub rotation:&'a str
}