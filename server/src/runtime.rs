// SPDX-FileCopyrightText: © 2021 ChiselStrike <info@chiselstrike.com>

use crate::api::ApiService;
use crate::policies::{FieldPolicies, LabelPolicies};
use crate::query::{MetaService, QueryEngine};
use crate::types::{ObjectType, TypeSystem};
use async_mutex::{Mutex, MutexGuardArc};
use once_cell::sync::OnceCell;
use std::sync::Arc;

pub(crate) struct Runtime {
    pub(crate) api: Arc<Mutex<ApiService>>,
    pub(crate) query_engine: QueryEngine,
    pub(crate) meta: MetaService,
    pub(crate) type_system: TypeSystem,
    pub(crate) policies: LabelPolicies,
}

impl Runtime {
    pub(crate) fn new(
        api: Arc<Mutex<ApiService>>,
        query_engine: QueryEngine,
        meta: MetaService,
        type_system: TypeSystem,
    ) -> Self {
        Self {
            api,
            query_engine,
            meta,
            type_system,
            policies: LabelPolicies::default(),
        }
    }

    /// Adds the current policies of ty to policies.
    pub(crate) fn get_policies(
        &self,
        ty: &ObjectType,
        policies: &mut FieldPolicies,
        current_path: &str,
    ) {
        for fld in &ty.fields {
            for lbl in &fld.labels {
                if let Some(p) = self.policies.get(lbl) {
                    if !p.except_uri.is_match(current_path) {
                        policies.insert(fld.name.clone(), p.transform);
                    }
                }
            }
        }
    }
}

thread_local!(static RUNTIME: OnceCell<Arc<Mutex<Runtime>>> = OnceCell::new());

pub(crate) fn set(rt: Runtime) {
    RUNTIME.with(|x| {
        x.set(Arc::new(Mutex::new(rt)))
            .map_err(|_| ())
            .expect("Runtime is already initialized.");
    })
}

pub(crate) async fn get() -> MutexGuardArc<Runtime> {
    let x = RUNTIME.with(|x| x.get().expect("Runtime is not yet initialized.").clone());
    x.lock_arc().await
}
