# create-users-from-org

## Description

Use the action [Load users](https://github.com/kudos-ink/create-users-from-org/actions/workflows/load_users.yml) to create all the members of a GitHub Organization in [Issues-API](https://api.morekudos.com). 

Note: The token used in the pipeline needs to belong to the organization and has the scope `read:org`. If not, only public members will be used.