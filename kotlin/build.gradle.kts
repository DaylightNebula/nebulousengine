plugins {
    kotlin("js") version "1.8.20"
}

repositories {
    mavenCentral()
}

dependencies {}

kotlin {
    js(IR) {
        browser {}
        binaries.executable()
    }
}