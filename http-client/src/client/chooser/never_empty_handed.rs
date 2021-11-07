use super::{
    super::super::regions::IpAddrWithPort, ChooseOptions, Chooser, ChooserFeedback, ChosenResults,
};
use num_rational::Ratio;
use rand::{prelude::*, thread_rng};

#[cfg(feature = "async")]
use futures::future::BoxFuture;

const DEFAULT_RANDOM_CHOOSE_RATIO: Ratio<usize> = Ratio::new_raw(1, 2);

#[derive(Debug, Clone)]
pub struct NeverEmptyHandedChooser<C> {
    inner_chooser: C,
    random_choose_ratio: Ratio<usize>,
}

impl<C> NeverEmptyHandedChooser<C> {
    #[inline]
    pub fn new(chooser: C, random_choose_ratio: Ratio<usize>) -> Self {
        Self {
            inner_chooser: chooser,
            random_choose_ratio,
        }
    }
}

impl<C: Default> Default for NeverEmptyHandedChooser<C> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default(), DEFAULT_RANDOM_CHOOSE_RATIO)
    }
}

impl<C: Chooser> Chooser for NeverEmptyHandedChooser<C> {
    #[inline]
    fn choose(&self, ips: &[IpAddrWithPort], opts: &ChooseOptions) -> ChosenResults {
        let chosen = self.inner_chooser.choose(ips, opts);
        if chosen.is_empty() {
            self.random_choose(ips).into()
        } else {
            chosen
        }
    }

    #[inline]
    fn feedback(&self, feedback: ChooserFeedback) {
        self.inner_chooser.feedback(feedback)
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    fn async_choose<'a>(
        &'a self,
        ips: &'a [IpAddrWithPort],
        opts: &'a ChooseOptions,
    ) -> BoxFuture<'a, ChosenResults> {
        Box::pin(async move {
            let chosen = self.inner_chooser.async_choose(ips, opts).await;
            if chosen.is_empty() {
                self.random_choose(ips).into()
            } else {
                chosen
            }
        })
    }

    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    fn async_feedback<'a>(&'a self, feedback: ChooserFeedback<'a>) -> BoxFuture<'a, ()> {
        self.inner_chooser.async_feedback(feedback)
    }
}

impl<C> NeverEmptyHandedChooser<C> {
    #[inline]
    fn random_choose(&self, ips: &[IpAddrWithPort]) -> Vec<IpAddrWithPort> {
        let chosen_len = (self.random_choose_ratio * ips.len()).ceil().to_integer();
        ips.choose_multiple(&mut thread_rng(), chosen_len)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::{ResponseError, ResponseErrorKind, RetriedStatsInfo},
            IpChooser,
        },
        *,
    };
    use qiniu_http::Extensions;
    use std::net::{IpAddr, Ipv4Addr};

    const IPS_WITHOUT_PORT: &[IpAddrWithPort] = &[
        IpAddrWithPort::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), None),
        IpAddrWithPort::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), None),
        IpAddrWithPort::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)), None),
    ];

    #[test]
    fn test_never_empty_handed_chooser() {
        env_logger::builder().is_test(true).try_init().ok();

        let ip_chooser: NeverEmptyHandedChooser<IpChooser> = Default::default();
        assert_eq!(
            ip_chooser
                .choose(IPS_WITHOUT_PORT, &Default::default())
                .into_ip_addrs(),
            IPS_WITHOUT_PORT.to_vec()
        );
        ip_chooser.feedback(ChooserFeedback::new(
            &[
                IpAddrWithPort::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), None),
                IpAddrWithPort::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), None),
            ],
            &RetriedStatsInfo::default(),
            &mut Extensions::default(),
            None,
            Some(&ResponseError::new(
                ResponseErrorKind::ParseResponseError,
                "Test Error",
            )),
        ));
        assert_eq!(
            ip_chooser
                .choose(IPS_WITHOUT_PORT, &Default::default())
                .into_ip_addrs(),
            [IpAddrWithPort::new(
                IpAddr::V4(Ipv4Addr::new(192, 168, 1, 3)),
                None
            )]
            .to_vec(),
        );

        ip_chooser.feedback(ChooserFeedback::new(
            IPS_WITHOUT_PORT,
            &RetriedStatsInfo::default(),
            &mut Extensions::default(),
            None,
            Some(&ResponseError::new(
                ResponseErrorKind::ParseResponseError,
                "Test Error",
            )),
        ));

        assert_eq!(
            ip_chooser
                .choose(IPS_WITHOUT_PORT, &Default::default())
                .into_ip_addrs()
                .len(),
            2
        );
    }
}
