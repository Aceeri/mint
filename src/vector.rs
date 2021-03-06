macro_rules! vec {
    ($name:ident [ $($field:ident = $index:expr),* ] = $fixed:ty) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : T, )*
        }

        impl<T: Clone> From<$fixed> for $name<T> {
            fn from(v: $fixed) -> Self {
                $name {
                    $(
                        $field: v[$index].clone(),
                    )*
                }
            }
        }

        impl<T> Into<$fixed> for $name<T> {
            fn into(self) -> $fixed {
                [$( self.$field.into() ),*]
            }
        }
    }
}

macro_rules! from {
    ($name:ident [ $($field:ident),* ] = $other:ident) => {
        impl<T> From<$other<T>> for $name<T> {
            fn from(v: $other<T>) -> Self {
                $name {
                    $(
                        $field: v.$field,
                    )*
                }
            }
        }
    }
}

vec!( Vector2 [x=0, y=1] = [T; 2] );
from!( Vector2 [x,y] = Point2 );
vec!( Vector3 [x=0, y=1, z=2] = [T; 3] );
from!( Vector3 [x,y,z] = Point3 );
vec!( Vector4 [x=0, y=1, z=2, w=3] = [T; 4] );
vec!( Point2 [x=0, y=1] = [T; 2] );
from!( Point2 [x,y] = Vector2 );
vec!( Point3 [x=0, y=1, z=2] = [T; 3] );
from!( Point3 [x,y,z] = Vector3 );
