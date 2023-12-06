multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EgldWrapperActionsModule {
    fn call_wrap_egld(&self, egld_amount: BigUint) -> EsdtTokenPayment {
        let wrapper_sc_address = self.egld_wrapper_address().get();
        self.egld_wrapper_proxy(wrapper_sc_address)
            .wrap_egld()
            .with_egld_transfer(egld_amount)
            .execute_on_dest_context()
    }

    #[storage_mapper("egldWrapperAddress")]
    fn egld_wrapper_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn egld_wrapper_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> multiversx_wegld_swap_sc::Proxy<Self::Api>;
}
