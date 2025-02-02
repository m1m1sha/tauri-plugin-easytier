plugins {
    // if edit kt file:
    id("com.android.library") version "8.1.1" apply true
    id("org.jetbrains.kotlin.android") version "1.9.10" apply true
    // else if tauri dev:
    // id("com.android.library")
    // id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.plugin.easytier"
    compileSdk = 33

    defaultConfig {
        minSdk = 21

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

dependencies {

    implementation("androidx.core:core-ktx:1.10.1")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.10.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
    implementation(project(":tauri-android"))
    // if edit kt file:
    implementation("com.fasterxml.jackson.core:jackson-databind:2.15.3") // 添加 com.fasterxml.jackson 的依赖项
}

