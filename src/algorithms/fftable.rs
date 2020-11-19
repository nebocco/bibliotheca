/// 高次の 2 冪根を持つ自然数で除算可能な [`Ring`](../type_traits/index.html) です。
pub trait Fftable: Ring + Copy {
    /// 高次の 2 冪根です。
    fn root() -> Self;

    /// [`root`](trait.Fftable.html#tymethod.root) の返す 2 冪根の逆数です。
    fn root_inv() -> Self;

    /// [`root`](trait.Fftable.html#tymethod.root) の返す 2 冪根の位数のログです。
    fn lg_ord() -> usize;

    /// 自然数による除算です。
    ///
    /// なお、assign しかないのはたまたまそれしか使わないからです。
    fn div_assign_by_usize(&mut self, den: usize);

    /// [`root`] や [`root_inv`] の返す 2 冪根を 2 乗していってできる列の逆順です。
    ///
    /// # Examples
    ///
    /// ```
    /// use fft::Fftable;
    /// type Fp = fp::F998244353;
    /// let a = Fp::root_seq::<fft::Forward>();
    /// assert_eq!(a[0], Fp::new(1));
    /// assert_eq!(a[1], Fp::new(-1));
    /// ```
    ///
    /// # `Tag` について
    ///
    /// [`Forward`] を使うと [`root`] が、[`Backward`] を使うと [`root_inv`] が呼ばれます。
    ///
    ///
    /// [`root`]: trait.Fftable.html#tymethod.root
    /// [`root_inv`]: trait.Fftable.html#tymethod.root_inv
    /// [`Forward`]: struct.Forward.html
    /// [`Backward`]: struct.Backward.html
    fn root_seq<Tag: DirectionTag>() -> Vec<Self> {
        let mut root = Tag::root::<Self>();
        let mut res = Vec::with_capacity(Self::lg_ord());
        for _ in 0..Self::lg_ord() {
            res.push(root);
            root *= root;
        }
        res.push(root);
        res.reverse();
        res
    }
}


use fp::F998244353;

impl Fftable for F998244353 {
    fn root() -> F998244353 {
        F998244353::new(3).pow(7 * 17)
    }
    fn root_inv() -> F998244353 {
        F998244353::root().inv()
    }
    fn lg_ord() -> usize {
        23
    }
    fn div_assign_by_usize(&mut self, den: usize) {
        *self /= F998244353::new(den as i64)
    }
}