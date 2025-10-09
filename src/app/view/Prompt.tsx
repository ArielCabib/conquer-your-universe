import { ORBIT_03 } from "../../constants";

interface BuildPromptProps {
  shouldShow: boolean;
}

export function BuildPrompt({ shouldShow }: BuildPromptProps) {
  if (!shouldShow) {
    return null;
  }

  return (
    <div
      style={{
        padding: "0.6rem 1rem",
        borderRadius: "0.75rem",
        background: "rgba(0,0,0,0.35)",
        border: "1px solid rgba(248,225,200,0.35)",
        color: ORBIT_03,
        fontFamily: "'Trebuchet MS', sans-serif",
        fontSize: "0.95rem",
        letterSpacing: "0.04em",
        textTransform: "uppercase",
      }}
    >
      Right click the planet to build a house
    </div>
  );
}
