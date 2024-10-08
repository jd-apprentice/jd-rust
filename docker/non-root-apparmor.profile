include <tunables/global>
include <abstractions/base>

/app {
	# Deny privilege escalation
	deny capability sys_admin,
	# Allow execution of any file in the app directory
	/app/** ix,
}