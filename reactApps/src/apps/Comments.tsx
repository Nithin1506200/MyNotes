import React from "react";
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type comments = {
  comment: string;
  children: Array<comments>;
};
const defautComment = {
  comment: "Lorem ipsum",
  children: [],
};

function deleteIComment(i: number, arr: comments[]) {
  return [...arr.slice(0, i), ...arr.slice(i + 1)];
}
function onChangeIComment(i: number, prev: comments[], comments: comments) {
  const prev_ = [...prev];
  prev_[i] = comments;
  return prev_;
}
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const CommentBox = (props: {
  comments: comments;
  onChangeComment: (comment: comments) => void;
  delete: () => void;
}) => {
  return (
    <div>
      <input
        value={props.comments.comment}
        onChange={(e) =>
          props.onChangeComment({ ...props.comments, comment: e.target.value })
        }
      ></input>
      {props.comments.children.map((e, i) => (
        <CommentBox
          comments={e}
          key={i}
          onChangeComment={(comments_) => {
            props.onChangeComment({
              ...props.comments,
              children: onChangeIComment(i, props.comments.children, comments_),
            });
            // onChangeIComment(i, setComments, comments);
          }}
          delete={() => {
            props.onChangeComment({
              ...props.comments,
              children: deleteIComment(i, props.comments.children),
            });
          }}
        />
      ))}
      <button
        onClick={() => {
          props.delete();
        }}
      >
        delete
      </button>
    </div>
  );
};
export default function Comments() {
  const [comments, setComments] = React.useState<Array<comments>>([
    defautComment,
  ]);
  return (
    <>
      <h1>Comments</h1>
      {comments.map((e, i) => (
        <CommentBox
          comments={e}
          key={i}
          onChangeComment={(comments) => {
            setComments((prev) => onChangeIComment(i, prev, comments));
          }}
          delete={() => {
            setComments((prev) => {
              return deleteIComment(i, prev);
            });
          }}
        />
      ))}
    </>
  );
}
