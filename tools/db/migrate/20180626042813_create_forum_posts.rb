class CreateForumPosts < ActiveRecord::Migration[5.2]
  def change
    create_table :forum_posts do |t|
      t.references :user, null: false
      t.references :topic, null: false
      t.references :post
      t.text :body, null: false
      t.string :media_type, null: false, limit: 8
      t.datetime :deleted_at
      t.timestamps
    end

    create_table :forum_posts_badges do |t|
      t.references :badage, null: false
      t.references :post, null: false
    end
    add_index :forum_posts_badges, %i[badage_id post_id], unique: true
  end
end