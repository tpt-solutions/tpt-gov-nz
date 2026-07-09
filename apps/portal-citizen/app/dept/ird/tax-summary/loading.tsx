export default function TaxSummaryLoading() {
  return (
    <main>
      <div style={{ height: "1rem", width: "6rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "1rem" }} />
      <div style={{ height: "2rem", width: "10rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "1rem" }} />
      <section style={{ marginBottom: "2rem" }}>
        <div style={{ height: "1.5rem", width: "12rem", background: "#e5e7eb", borderRadius: "0.25rem", marginBottom: "0.75rem" }} />
        {[1, 2, 3, 4, 5].map((i) => (
          <div key={i} style={{ height: "1rem", width: `${80 - i * 10}%`, background: "#f3f4f6", borderRadius: "0.25rem", marginBottom: "0.5rem" }} />
        ))}
      </section>
    </main>
  );
}
