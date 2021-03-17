fn read_msg_from_mbx(&self, msg: &mut [u32]) -> Result<(), Box<dyn Error>> {
    let len = min(msg.len(), self.mbx.borrow().size as usize);

    self.obtain_mbx_lock()?;

    for (idx, el) in msg[0..len].iter_mut().enumerate() {
        *el = self.get_reg32_array(IXGBE_VFMBMEM, idx as u32);
    }

    self.set_reg32(IXGBE_VFMAILBOX, IXGBE_VFMAILBOX_ACK);

    Ok(())
}
