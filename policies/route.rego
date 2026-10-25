package route_foundry.route
default allow := false
allow if {
 input.required_region == input.provider_region
 not restricted_external
}
restricted_external if {
 input.data_class == "restricted"
 input.external == true
}
