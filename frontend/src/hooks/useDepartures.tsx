import useSWR from "swr";

export async function useDepartures(baseUrl: string) {
  const { data, error, isLoading } = useSWR("/api/user", (args) =>
    fetch(args).then((res) => res.json())
  );
}
