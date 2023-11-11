#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StringTypes
{
    SlashSeparatedStringVec(SlashSeparatedStringVec),
    Array(Vec<String>)
}

const delimiter:String="/".to_string();

#[derive(Debug, PartialEq)]
struct SlashSeparatedStringVec
{
    values:Vec<String>
}

impl Serialize for SlashSeparatedStringVec
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            for value of &self.values
            {
                serializer.serialize_string(value+delimiter)
            }
    }
}

struct StringStringVisitor;
impl<'de> Visitor<'de> for StringStringVisitor {
    type Value = Timespan;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A set of strings separated by ".to_string() + delimiter)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut vec:Vec<NaiveTime>=Vec::new();
        let values=value.split(delimiter);
        
        Ok(
            SlashSeparatedStringVec{
                value:values
            }
        )
    }
}

impl<'de> Deserialize<'de> for SlashSeparatedStringVec
{
    fn deserialize<D>(deserializer:D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(deserializer.deserialize_str(NaiveTimeVisitor)?)
    }
}