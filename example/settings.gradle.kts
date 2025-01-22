pluginManagement {
    repositories {
        maven("https://maven.aliyun.com/repository/public/")
        google {
            content {
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }
        }
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
      maven("https://maven.aliyun.com/repository/public/")
        mavenCentral()
        google()
        maven("https://plugins.gradle.org/m2/")
    }
}

rootProject.name = "android-rust-example"
include(":app")

includeBuild("..") {
    dependencySubstitution {
        substitute(module("io.github.MatrixDev.android-rust:plugin"))
    }
}
