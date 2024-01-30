use icrate::Foundation::NSObject;
use objc2::rc::Id;
use objc2::{extern_class, extern_methods, mutability, ClassType};

extern_class!(
    /// An object that stores color data and sometimes opacity (alpha value).
    ///
    /// <https://developer.apple.com/documentation/appkit/nscolor?language=objc>
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIColor;

    unsafe impl ClassType for UIColor {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

// SAFETY: Documentation clearly states:
// > Color objects are immutable and thread-safe
unsafe impl Send for UIColor {}
unsafe impl Sync for UIColor {}

extern_methods!(
    unsafe impl UIColor {
        #[method_id(clearColor)]
        pub fn clear() -> Id<Self>;
    }
);
