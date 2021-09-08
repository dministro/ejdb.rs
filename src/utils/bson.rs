use bson::Bson;

pub trait BsonNumber {
    fn to_bson(self) -> Bson;
}

impl BsonNumber for f32 {
    #[inline]
    fn to_bson(self) -> Bson {
        #[cfg(feature = "bson_0_13")]
        let ret = Bson::FloatingPoint(self as f64);
        #[cfg(feature = "bson_1_2")]
        let ret = Bson::Double(self as f64);

        ret
    }
}

impl BsonNumber for f64 {
    #[inline]
    fn to_bson(self) -> Bson {
        #[cfg(feature = "bson_0_13")]
        let ret = Bson::FloatingPoint(self);
        #[cfg(feature = "bson_1_2")]
        let ret = Bson::Double(self);

        ret
    }
}

impl BsonNumber for i32 {
    #[inline]
    fn to_bson(self) -> Bson {
        #[cfg(feature = "bson_0_13")]
        let ret = Bson::I32(self);
        #[cfg(feature = "bson_1_2")]
        let ret = Bson::Int32(self);

        ret
    }
}

impl BsonNumber for i64 {
    #[inline]
    fn to_bson(self) -> Bson {
        #[cfg(feature = "bson_0_13")]
        let ret = Bson::I64(self);
        #[cfg(feature = "bson_1_2")]
        let ret = Bson::Int64(self);

        ret
    }
}
