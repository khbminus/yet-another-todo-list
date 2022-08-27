/**
* JetBrains Space Automation
* This Kotlin-script file lets you automate build activities
* For more info, see https://www.jetbrains.com/help/space/automation.html
*/

job("Build and run tests") {
    container(displayName = "Rust build", image = "rust:latest") {
    	shellScript {
            content = """
            cargo install names
            """
        }
    }
        container(displayName = "termtree", image = "rust:latest") {
            shellScript {
                content = """
                names
                """
            }
    }
}
