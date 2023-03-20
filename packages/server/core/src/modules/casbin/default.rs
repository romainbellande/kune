use super::{
    Permission::{self, Archive, Delete, Edit, Read, Write},
    Resource,
};
use crate::utils::enum_string;
use std::str::FromStr;

#[derive(Clone)]
pub struct ResourcePolicies {
    pub resource: Resource,
    pub permissions: Vec<Permission>,
}

impl ResourcePolicies {
    pub fn new(resource: Resource, permissions: Vec<Permission>) -> Self {
        Self {
            resource,
            permissions,
        }
    }

    pub fn into_casbin_policies(&self, role: String, gid: String) -> Vec<Vec<String>> {
        self.permissions
            .clone()
            .into_iter()
            .map(|permission| {
                vec![
                    role.clone(),
                    gid.clone(),
                    self.resource.to_string(),
                    permission.to_string(),
                ]
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct Policies {
    pub role: String,
    pub policies: Vec<ResourcePolicies>,
}

impl Policies {
    pub fn new(role: String) -> Self {
        Self {
            role,
            policies: vec![],
        }
    }

    pub fn add(&mut self, resource: Resource, permissions: Vec<Permission>) -> &mut Self {
        let resource_policies = ResourcePolicies::new(resource, permissions);
        self.policies.push(resource_policies);
        self
    }

    pub fn into_casbin_policies(&self, gid: String) -> Vec<Vec<String>> {
        self.policies
            .clone()
            .into_iter()
            .flat_map(|policy| policy.into_casbin_policies(self.role.clone(), gid.clone()))
            .collect()
    }
}

enum_string!(DefaultRole, [SuperAdmin, Admin, Editor, Member]);

pub fn get_default_policies() -> Vec<Policies> {
    let member_policies: Policies = Policies::new(DefaultRole::Member.to_string())
        .add(Resource::Group, vec![Read])
        .add(Resource::User, vec![Read])
        .add(Resource::Referendum, vec![Read])
        .to_owned();

    let editor_policies: Policies = Policies::new(DefaultRole::Editor.to_string())
        .add(Resource::Group, vec![Read, Edit])
        .add(Resource::User, vec![Read, Edit])
        .add(Resource::Referendum, vec![Read, Edit])
        .to_owned();

    let admin_policies: Policies = Policies::new(DefaultRole::Admin.to_string())
        .add(Resource::Group, vec![Read, Edit])
        .add(Resource::User, vec![Read, Edit, Write, Delete])
        .add(
            Resource::Referendum,
            vec![Read, Edit, Write, Delete, Archive],
        )
        .to_owned();

    vec![member_policies, editor_policies, admin_policies]
}
