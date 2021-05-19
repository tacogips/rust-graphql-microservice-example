import React from "react";
import { useQuery, gql } from "@apollo/client";

const GET_ARTICLES = gql`
  query {
    findArticles {
      id
      author {
        id
        name
      }
      status
      overview
    }
  }
`;

const ArticleList = () => {
  const { loading, error, data } = useQuery(GET_ARTICLES);
  if (loading) return <div>Loading...</div>;
  if (error) {
    console.error(error);
    return <div>Error </div>;
  }

  return (
    <div>
      <header className="mt-10"></header>
      <main>
        {data["findArticles"].map((article: any) => {
          return (
            <div key={article.id}>
              <section className="flex items-center justify-center px-4 bg-white">
                <div className="max-w-xl w-full rounded-lg shadow-lg p-4 flex md:flex-row flex-col">
                  <div className="flex-1">
                    <p className="text-gray-500 my-1 whitespace-normal ">
                      {article.overview.replace("\\n", "")}
                    </p>
                  </div>
                  <div className="md:px-2 mt-3 md:mt-0 items-center flex">
                    <button className="bg-blue-500 text-white font-bold px-4 py-2 text-sm uppercase rounded tracking-wider focus:outline-none hover:bg-blue-600">
                      {" "}
                      more
                    </button>
                  </div>
                </div>
              </section>
            </div>
          );
        })}
      </main>
    </div>
  );
};

export default ArticleList;
