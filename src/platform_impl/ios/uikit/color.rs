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

        #[method_id(redColor)]
        pub fn red() -> Id<Self>;

        #[method_id(orangeColor)]
        pub fn orange() -> Id<Self>;

        #[method_id(yellowColor)]
        pub fn yellow() -> Id<Self>;

        #[method_id(greenColor)]
        pub fn green() -> Id<Self>;

        #[method_id(blueColor)]
        pub fn bleu() -> Id<Self>;

        #[method_id(cyanColor)]
        pub fn cyan() -> Id<Self>;

        #[method_id(blackColor)]
        pub fn black() -> Id<Self>;

        #[method_id(brownColor)]
        pub fn brown() -> Id<Self>;

        #[method_id(grayColor)]
        pub fn gray() -> Id<Self>;

        #[method_id(purpleColor)]
        pub fn purple() -> Id<Self>;

        #[method_id(magentaColor)]
        pub fn magenta() -> Id<Self>;
    }
);
