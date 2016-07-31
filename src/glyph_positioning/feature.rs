//! The features.

use truetype::Tag;

use {Result, Tape, Value};

table! {
    @define
    #[doc = "A feature list."]
    pub Features {
        count   (u16         ), // FeatureCount
        headers (Vec<Header> ), // FeatureRecord
        records (Vec<Feature>),
    }
}

table! {
    #[doc = "The header of a feature-list record."]
    #[derive(Copy)]
    pub Header {
        tag    (Tag), // FeatureTag
        offset (u16), // Feature
    }
}

table! {
    @define
    #[doc = "A feature."]
    pub Feature {
        parameter_offset (u16            ), // FeatureParams
        lookup_count     (u16            ), // LookupCount
        lookup_indices   (Vec<u16>       ), // LookupListIndex
        parameters       (Option<Vec<u8>>),
    }
}

impl Value for Features {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let count = read_value!(tape, u16);
        let headers = read_walue!(tape, count as usize, Vec<Header>);
        let mut records: Vec<Feature> = Vec::with_capacity(count as usize);
        for i in 0..(count as usize) {
            try!(tape.jump(position + headers[i].offset as u64));
            records.push(read_value!(tape));
        }
        Ok(Features { count: count, headers: headers, records: records })
    }
}

impl Value for Feature {
    fn read<T: Tape>(tape: &mut T) -> Result<Self> {
        let position = try!(tape.position());
        let parameter_offset = read_value!(tape);
        let lookup_count = read_value!(tape);
        let lookup_indices = read_walue!(tape, lookup_count as usize);
        let parameters = if parameter_offset != 0 {
            try!(tape.jump(position + parameter_offset as u64));
            Some(read_bytes!(tape, 0))
        } else {
            None
        };
        Ok(Feature {
            parameter_offset: parameter_offset,
            lookup_count: lookup_count,
            lookup_indices: lookup_indices,
            parameters: parameters,
        })
    }
}
