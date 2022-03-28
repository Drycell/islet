use rmm_core::error::{Error, ErrorKind};
use rmm_core::realm::vcpu::VCPU;
use rmm_core::realm::vm::VM;
use rmm_core::realm::vmem::IPATranslation;

use super::context::Context;
use super::mm::stage2_translation::Stage2Translation;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::mutex::Mutex;
use spinning_top::Spinlock;

type VMMap = BTreeMap<usize, Arc<Mutex<VM<Context>>>>;

static VMS: Spinlock<(usize, VMMap)> = Spinlock::new((0, BTreeMap::new()));

pub fn new(num_vcpu: usize) -> Arc<Mutex<VM<Context>>> {
    let mut vms = VMS.lock();

    let id = vms.0;

    let s2_table = Arc::new(Mutex::new(
        Box::new(Stage2Translation::new()) as Box<dyn IPATranslation>
    ));

    let vttbr = s2_table.lock().get_vttbr(id);
    let vm = VM::new(id, num_vcpu, s2_table);

    vm.lock()
        .vcpus
        .iter()
        .for_each(|vcpu: &Arc<Mutex<VCPU<Context>>>| vcpu.lock().context.sys_regs.vttbr = vttbr);

    vms.0 += 1;
    vms.1.insert(id, vm.clone());

    vm
}

pub fn get(id: usize) -> Option<Arc<Mutex<VM<Context>>>> {
    VMS.lock().1.get(&id).map(|vm| Arc::clone(vm))
}

pub fn remove(id: usize) -> Result<(), Error> {
    VMS.lock()
        .1
        .remove(&id)
        .ok_or(Error::new(ErrorKind::NotConnected))?;
    Ok(())
}
