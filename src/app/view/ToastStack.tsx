import { ToastMessage } from "../types";

interface ToastStackProps {
  toasts: ToastMessage[];
}

export function ToastStack({ toasts }: ToastStackProps) {
  if (toasts.length === 0) {
    return null;
  }

  return (
    <div className="pointer-events-none fixed inset-x-0 top-6 flex justify-center">
      <div className="flex flex-col gap-3">
        {toasts.map((toast) => (
          <div
            key={toast.id}
            className="rounded-2xl border border-orbit-03/40 bg-panel-soft px-4 py-2 font-trebuchet text-[0.95rem] tracking-[0.04em] text-orbit-03 shadow-context-menu"
          >
            {toast.message}
          </div>
        ))}
      </div>
    </div>
  );
}
