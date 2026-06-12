# Agent instructions (Relay monorepo)

## Production servers (Linode masters)

For **deploying**, **restarting**, **logs**, **tunnels**, and **HTTP/Git debugging** on **relay-gateway** production hosts (e.g. **atlanta2** Linode, **dfw1** Rackspace Spot VM):

- **Provision / first install:** Terraform + **[`relay-gateway/docs/RELAY_INFRA_OPS.md`](relay-gateway/docs/RELAY_INFRA_OPS.md)** — **`terraform/linode-relay`**, **`terraform/rackspace-spot-vm`**, **`./scripts/relay-master-deploy.sh`**, **`relay-master-validate.sh`**. Linode provider: [registry](https://registry.terraform.io/providers/linode/linode/latest/docs).
- **Ongoing updates:** from **`relay-gateway/`**, **`./scripts/relay-fleet.sh`** for **`deploy-all`**, **`status`**, **`drift`**, **`ssh-check`**, **`repair`**, **`features-set`**, **`reclone-repo`**, etc. See **[`relay-gateway/docs/RELAY_FLEET_CONSOLE.md`](relay-gateway/docs/RELAY_FLEET_CONSOLE.md)** ( **`--build` before `update`/`deploy-all`**, stale tarball pitfall). Local **`relay-gateway/relay.env`** (gitignored): **`RELAY_OPERATOR_FLEET`** or **`RELAY_MASTER_PEER_LIST`**.
- Then follow **`relay-gateway/.cursor/rules/relay-production-ops.mdc`** (SSH / **autossh**, tarball + `install.sh update`, POST-only `/api/config` and `/api/status`). The monorepo root **`.cursor/rules/relay-production-ops.mdc`** is a short pointer to the same content. **Helm-style reconcile:** **`relay-gateway/docs/RELAY_RECONCILE_PLAN.md`**.

GitHub Copilot and other tools that read this file should treat that rule as the source of truth for production operations.

## relay-gateway code

- Main crate and scripts live under **`relay-gateway/`**.
- Run tests from **`relay-gateway/`**: `cargo test`.
- Fleet **`pre-receive`** branch policy (`.relay.yaml`), forge vs mirror pitfalls, and **`relay-branch-policy-check`**: **[`relay-gateway/docs/RELAY_BRANCH_POLICY_OPERATOR.md`](relay-gateway/docs/RELAY_BRANCH_POLICY_OPERATOR.md)**.
- CleverScript React-like rendering goals (infrequent re-renders, native-heavy UI, handler registry policy): **[`relay-gateway/docs/RELAY_CLEVERSCRIPT_REACT_RENDERING.md`](relay-gateway/docs/RELAY_CLEVERSCRIPT_REACT_RENDERING.md)**.
- Docker / GitLab CI / Rackspace image: **[`relay-gateway/docs/DOCKER_GITLAB_RACKSPACE.md`](relay-gateway/docs/DOCKER_GITLAB_RACKSPACE.md)**. IPFS hardening env: **[`relay-gateway/docs/RELAY_IPFS.md`](relay-gateway/docs/RELAY_IPFS.md)**.
