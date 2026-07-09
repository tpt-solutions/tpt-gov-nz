export default function KiwiSaverLoading() {
  return (
    <main>
      <div style={{ height: "1rem", width: "6rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "1rem" }} />
      <div style={{ height: "2rem", width: "8rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "1rem" }} />
      {[1, 2, 3].map((i) => (
        <section key={i} style={{ marginBottom: "2rem" }}>
          <div style={{ height: "1.5rem", width: "10rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "0.75rem" }} />
          {[1, 2].map((j) => (
            <div key={j} style={{ height: "1rem", width: `${70 - j * 10}%`, background: "#f3f4f6", borderRadius: "0.25rem", marginBottom: "0.5rem" }} />
          ))}
        </section>
      ))}
    </main>
  );
}
