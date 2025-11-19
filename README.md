# AAR Sync

Android 라이브러리와 샘플 앱 개발 시 AAR 파일 동기화를 자동화하는 CLI 도구입니다.

## 문제 상황

Android 라이브러리와 샘플 앱을 별도 프로젝트로 관리할 때 다음과 같은 반복 작업이 필요합니다:

1. 라이브러리 프로젝트에서 AAR 파일 빌드
2. 생성된 AAR 파일을 샘플 앱의 libs 디렉토리로 복사
3. 샘플 앱에서 clean build 수행
4. Gradle sync 후 앱 실행

이러한 과정을 하루에 수십 번 반복하면 상당한 시간이 소모됩니다.

## 해결 방법

AAR Sync는 위 과정을 단일 명령으로 자동화하며, 다음과 같은 최적화를 제공합니다:

- AAR 파일 해시 비교를 통한 불필요한 빌드 스킵
- 빌드 시간 측정 및 시각적 피드백
- 단계별 오류 처리 및 명확한 에러 메시지
- 선택적 clean 및 부분 빌드 지원

## 주요 기능

- 라이브러리 프로젝트 자동 빌드
- AAR 파일 변경 감지 및 선택적 복사
- 샘플 앱 clean 및 빌드 자동화
- 빌드 시간 측정
- 컬러 터미널 출력으로 진행 상황 표시
- TOML 기반 설정 파일 관리

## 설치

### 필수 요구사항

- Rust 1.70 이상
- Android 프로젝트에 gradlew 실행 권한

### 빌드 및 설치
```bash
git clone https://github.com/yourusername/aar-sync.git
cd aar-sync
cargo install --path .
```

설치 후 `aar-sync` 명령을 어디서든 실행할 수 있습니다.

## 설정

프로젝트 루트에 `config.toml` 파일을 생성합니다:
```toml
[library]
project_path = "/absolute/path/to/library-project"
module_name = "library"
build_variant = "release"

[sample]
project_path = "/absolute/path/to/sample-app"
libs_path = "app/libs"
build_variant = "debug"
```

### 설정 항목 설명

#### library 섹션
- `project_path`: 라이브러리 프로젝트의 절대 경로
- `module_name`: AAR을 생성할 모듈 이름 (일반적으로 "library")
- `build_variant`: 빌드 타입 ("release" 또는 "debug")

#### sample 섹션
- `project_path`: 샘플 앱 프로젝트의 절대 경로
- `libs_path`: AAR 파일을 복사할 샘플 앱 내 상대 경로
- `build_variant`: 샘플 앱 빌드 타입

## 사용 방법

### 기본 실행
```bash
aar-sync
```

다음 작업을 순차적으로 수행합니다:

1. 라이브러리 빌드 (assembleRelease)
2. AAR 파일 해시 비교 및 복사 (변경된 경우만)
3. 샘플 앱 clean
4. 샘플 앱 빌드 (assembleDebug)

AAR 파일이 변경되지 않았다면 샘플 앱 빌드를 자동으로 스킵합니다.

### 커맨드 옵션
```bash
# 커스텀 설정 파일 사용
aar-sync --config custom-config.toml

# 강제 동기화 (해시 비교 무시)
aar-sync --force

# clean 단계 스킵
aar-sync --skip-clean

# 라이브러리만 빌드 (샘플 앱 작업 생략)
aar-sync --library-only
```

### 실행 예시
```
=== AAR Sync Tool ===
Building library...
✓ Library build completed (3421ms)
Copying AAR file...
✓ AAR copied /path/to/library.aar -> /path/to/sample/app/libs/library.aar
Cleaning sample app...
✓ Clean completed
Building sample app...
✓ Sample build completed (2156ms)

✓ All tasks completed successfully!
```

## 고급 사용

### 프로젝트별 설정 관리

여러 프로젝트를 작업하는 경우 프로젝트별 설정 파일을 생성하고 전환할 수 있습니다:
```bash
aar-sync --config project-a.toml
aar-sync --config project-b.toml
```

### 빠른 반복 개발

UI만 수정하는 경우 라이브러리 빌드만 수행하고 직접 확인할 수 있습니다:
```bash
aar-sync --library-only
# 별도로 Android Studio에서 샘플 앱 실행
```

### 성능 최적화

clean 단계는 대부분의 경우 불필요합니다. 빌드 속도를 높이려면:
```bash
aar-sync --skip-clean
```

## 프로젝트 구조
```
aar-sync/
├── src/
│   ├── main.rs          # CLI 진입점 및 전체 워크플로우
│   ├── config.rs        # TOML 설정 파일 파싱
│   ├── builder.rs       # Gradle 빌드 명령 실행
│   └── file_sync.rs     # AAR 파일 동기화 및 해시 비교
├── config.toml          # 설정 파일 예시
├── Cargo.toml
└── README.md
```

## 의존성

- `clap`: CLI 인자 파싱
- `anyhow`: 에러 처리
- `serde`, `toml`: 설정 파일 직렬화
- `sha2`: AAR 파일 해시 계산
- `colored`: 터미널 출력 색상화

## 향후 개발 계획

- Watch 모드: 파일 변경 자동 감지 및 빌드
- ADB 통합: 빌드 후 자동 설치 및 앱 실행
- 병렬 빌드: 독립적 작업 동시 실행
- 빌드 캐시: 증분 빌드 최적화
- 멀티 샘플 앱 지원: 여러 샘플 앱에 동시 배포

## 라이선스

MIT

## 기여

이슈와 Pull Request를 환영합니다.