#[macro_export]
macro_rules! impl_EnumVar {
(for $($t:ty),+) => {$(
impl Var for $t {


  fn get_property(&self) -> Self::Via {
    GString::from_str(self.into()).unwrap()
  }

  fn set_property(&mut self, value: Self::Via) {
    let value = value.to_string();
    let value = value.as_str();
    if let Ok(tile) = Self::from_str(value) {
      *self = tile;
    } else {
      godot_error!("Invalid Variant: {}", value);
    }
  }

  fn var_hint() -> PropertyHintInfo {
    PropertyHintInfo {
      hint: PropertyHint::ENUM,
      hint_string: GString::join(
        &",".into(),
        &PackedStringArray::from_iter(Self::iter().map(|tile| {
          let string: &str = tile.into();
          GString::from(string)
        })),
      ),
    }
  }
}

)*}}
