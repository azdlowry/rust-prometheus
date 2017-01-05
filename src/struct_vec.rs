build_struct_metric! {
    MetricName {
        metric_value1: {
            metric_value1: {

            }
        },
    }
}

macro_rules! map_field {
    ($key:ident, $value:block) => (
        $key
    )
}

macro_rules! divide_block {
    ({
        ($key:ident: $value:block),*
    }) => (
        $(map_field!($key, $value))
    )
}

macro_rules! build_struct_metric {
    ($name:ident {
        ($key:ident: $value:expr),*
    }) => {
        struct $name {
            $(map_field!($key, $value))*
        }
    }
}
