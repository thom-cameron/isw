{
  lib,
  fetchFromGitLab,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "isw";
  version = "0.3.0";

  src = fetchFromGitLab {
    owner = "thom-cameron";
    repo = pname;
    rev = version;
    hash = "sha256-8DbRTJheWt1hlwNW566rnw9hccXajEqhQ/IZsb8cDa4=";
  };

  cargoHash = "sha256-LpKiP7bwiNEUXYvQsUG//wqOwGmg2UD3DA0Gb2h7kMw=";

  meta = with lib; {
    description = "a simple terminal stopwatch application";
    homepage = "https://gitlab.com/thom-cameron/isw";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [thom-cameron];
  };
}
